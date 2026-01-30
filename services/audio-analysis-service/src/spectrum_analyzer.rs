use anyhow::{Result, Context};
use rustfft::{FftPlanner, num_complex::Complex};
use apodize::{hanning_iter, hamming_iter};
use std::f64::consts::PI;
use tracing::{info, debug};
use shared::AudioSpectrum;

/// Audio spectrum analyzer for frequency domain analysis
pub struct SpectrumAnalyzer {
    window_size: usize,
    overlap: f64,
    window_type: WindowType,
}

/// Window function types for spectral analysis
#[derive(Debug, Clone)]
pub enum WindowType {
    Hanning,
    Hamming,
    Rectangular,
    Blackman,
}

impl SpectrumAnalyzer {
    /// Create a new spectrum analyzer
    pub fn new(window_size: usize, overlap: f64, window_type: WindowType) -> Self {
        Self {
            window_size,
            overlap: overlap.clamp(0.0, 0.95),
            window_type,
        }
    }
    
    /// Create analyzer with default parameters optimized for speech analysis
    pub fn new_for_speech() -> Self {
        Self::new(2048, 0.5, WindowType::Hanning)
    }
    
    /// Analyze audio samples and return spectrum
    pub fn analyze(&self, samples: &[f32], sample_rate: u32) -> Result<AudioSpectrum> {
        if samples.is_empty() {
            return Err(anyhow::anyhow!("No audio samples provided"));
        }
        
        if samples.len() < self.window_size {
            return Err(anyhow::anyhow!(
                "Audio too short: {} samples (need at least {})",
                samples.len(),
                self.window_size
            ));
        }
        
        let hop_size = ((1.0 - self.overlap) * self.window_size as f64) as usize;
        let mut all_magnitudes = Vec::new();
        let mut frame_count = 0;
        
        // Process audio in overlapping windows
        let mut start = 0;
        while start + self.window_size <= samples.len() {
            let window_samples = &samples[start..start + self.window_size];
            let magnitudes = self.compute_fft_magnitudes(window_samples)?;
            
            if all_magnitudes.is_empty() {
                all_magnitudes = magnitudes;
            } else {
                // Average the magnitudes across frames
                for (i, mag) in magnitudes.iter().enumerate() {
                    all_magnitudes[i] += mag;
                }
            }
            
            frame_count += 1;
            start += hop_size;
        }
        
        // Average the accumulated magnitudes
        if frame_count > 1 {
            for mag in &mut all_magnitudes {
                *mag /= frame_count as f64;
            }
        }
        
        // Generate frequency bins
        let frequencies = self.generate_frequency_bins(sample_rate);
        
        debug!("Analyzed {} frames, {} frequency bins", frame_count, frequencies.len());
        
        Ok(AudioSpectrum {
            frequencies,
            magnitudes: all_magnitudes,
            sample_rate,
            window_size: self.window_size,
        })
    }
    
    /// Compute FFT magnitudes for a single window
    fn compute_fft_magnitudes(&self, samples: &[f32]) -> Result<Vec<f64>> {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.window_size);
        
        // Apply window function and convert to complex
        let windowed_samples = self.apply_window(samples);
        let mut complex_samples: Vec<Complex<f64>> = windowed_samples
            .into_iter()
            .map(|s| Complex::new(s as f64, 0.0))
            .collect();
        
        // Perform FFT
        fft.process(&mut complex_samples);
        
        // Compute magnitudes (only first half due to symmetry)
        let magnitudes: Vec<f64> = complex_samples
            .iter()
            .take(self.window_size / 2 + 1)
            .map(|c| c.norm())
            .collect();
        
        Ok(magnitudes)
    }
    
    /// Apply window function to samples
    fn apply_window(&self, samples: &[f32]) -> Vec<f32> {
        match self.window_type {
            WindowType::Hanning => {
                hanning_iter(samples.len())
                    .zip(samples.iter())
                    .map(|(w, &s)| s * w as f32)
                    .collect()
            }
            WindowType::Hamming => {
                hamming_iter(samples.len())
                    .zip(samples.iter())
                    .map(|(w, &s)| s * w as f32)
                    .collect()
            }
            WindowType::Rectangular => samples.to_vec(),
            WindowType::Blackman => {
                samples
                    .iter()
                    .enumerate()
                    .map(|(i, &s)| {
                        let n = samples.len() as f64;
                        let w = 0.42 - 0.5 * (2.0 * PI * i as f64 / (n - 1.0)).cos()
                            + 0.08 * (4.0 * PI * i as f64 / (n - 1.0)).cos();
                        s * w as f32
                    })
                    .collect()
            }
        }
    }
    
    /// Generate frequency bins for the spectrum
    fn generate_frequency_bins(&self, sample_rate: u32) -> Vec<f64> {
        let num_bins = self.window_size / 2 + 1;
        let freq_resolution = sample_rate as f64 / self.window_size as f64;
        
        (0..num_bins)
            .map(|i| i as f64 * freq_resolution)
            .collect()
    }
    
    /// Extract formant frequencies (important for speech analysis)
    pub fn extract_formants(&self, spectrum: &AudioSpectrum, num_formants: usize) -> Vec<f64> {
        let mut formants = Vec::new();
        
        // Simple peak picking algorithm for formant detection
        let min_freq_idx = (300.0 / (spectrum.sample_rate as f64 / spectrum.window_size as f64)) as usize;
        let max_freq_idx = (3000.0 / (spectrum.sample_rate as f64 / spectrum.window_size as f64)) as usize;
        
        let search_range = min_freq_idx..max_freq_idx.min(spectrum.magnitudes.len());
        
        // Find peaks in the spectrum
        let mut peaks = Vec::new();
        for i in search_range {
            if i > 0 && i < spectrum.magnitudes.len() - 1 {
                if spectrum.magnitudes[i] > spectrum.magnitudes[i - 1] 
                    && spectrum.magnitudes[i] > spectrum.magnitudes[i + 1] {
                    peaks.push((i, spectrum.magnitudes[i]));
                }
            }
        }
        
        // Sort peaks by magnitude and take the strongest ones
        peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (idx, _) in peaks.iter().take(num_formants) {
            if *idx < spectrum.frequencies.len() {
                formants.push(spectrum.frequencies[*idx]);
            }
        }
        
        formants.sort_by(|a, b| a.partial_cmp(b).unwrap());
        formants
    }
    
    /// Calculate spectral centroid (brightness measure)
    pub fn spectral_centroid(&self, spectrum: &AudioSpectrum) -> f64 {
        let mut weighted_sum = 0.0;
        let mut magnitude_sum = 0.0;
        
        for (freq, mag) in spectrum.frequencies.iter().zip(spectrum.magnitudes.iter()) {
            weighted_sum += freq * mag;
            magnitude_sum += mag;
        }
        
        if magnitude_sum > 0.0 {
            weighted_sum / magnitude_sum
        } else {
            0.0
        }
    }
    
    /// Calculate spectral rolloff (frequency below which 85% of energy is contained)
    pub fn spectral_rolloff(&self, spectrum: &AudioSpectrum, rolloff_percent: f64) -> f64 {
        let total_energy: f64 = spectrum.magnitudes.iter().map(|m| m * m).sum();
        let threshold = total_energy * rolloff_percent;
        
        let mut cumulative_energy = 0.0;
        for (i, mag) in spectrum.magnitudes.iter().enumerate() {
            cumulative_energy += mag * mag;
            if cumulative_energy >= threshold && i < spectrum.frequencies.len() {
                return spectrum.frequencies[i];
            }
        }
        
        spectrum.frequencies.last().copied().unwrap_or(0.0)
    }
    
    /// Calculate zero crossing rate (useful for voiced/unvoiced detection)
    pub fn zero_crossing_rate(&self, samples: &[f32]) -> f64 {
        if samples.len() < 2 {
            return 0.0;
        }
        
        let mut crossings = 0;
        for i in 1..samples.len() {
            if (samples[i] >= 0.0) != (samples[i - 1] >= 0.0) {
                crossings += 1;
            }
        }
        
        crossings as f64 / (samples.len() - 1) as f64
    }
}

impl Default for SpectrumAnalyzer {
    fn default() -> Self {
        Self::new_for_speech()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;
    
    fn generate_sine_wave(frequency: f32, sample_rate: u32, duration: f32) -> Vec<f32> {
        let num_samples = (sample_rate as f32 * duration) as usize;
        (0..num_samples)
            .map(|i| {
                let t = i as f32 / sample_rate as f32;
                (2.0 * PI * frequency * t).sin()
            })
            .collect()
    }
    
    #[test]
    fn test_spectrum_analyzer_creation() {
        let analyzer = SpectrumAnalyzer::new(1024, 0.5, WindowType::Hanning);
        assert_eq!(analyzer.window_size, 1024);
        assert_eq!(analyzer.overlap, 0.5);
    }
    
    #[test]
    fn test_sine_wave_analysis() {
        let analyzer = SpectrumAnalyzer::new_for_speech();
        let sample_rate = 44100;
        let test_frequency = 440.0; // A4 note
        
        // Generate a 1-second sine wave
        let samples = generate_sine_wave(test_frequency, sample_rate, 1.0);
        
        let spectrum = analyzer.analyze(&samples, sample_rate);
        assert!(spectrum.is_ok());
        
        let spectrum = spectrum.unwrap();
        assert_eq!(spectrum.sample_rate, sample_rate);
        assert!(!spectrum.frequencies.is_empty());
        assert!(!spectrum.magnitudes.is_empty());
        assert_eq!(spectrum.frequencies.len(), spectrum.magnitudes.len());
    }
    
    #[test]
    fn test_spectral_features() {
        let analyzer = SpectrumAnalyzer::new_for_speech();
        let sample_rate = 44100;
        
        // Generate a sine wave
        let samples = generate_sine_wave(1000.0, sample_rate, 0.5);
        let spectrum = analyzer.analyze(&samples, sample_rate).unwrap();
        
        // Test spectral centroid
        let centroid = analyzer.spectral_centroid(&spectrum);
        assert!(centroid > 0.0);
        
        // Test spectral rolloff
        let rolloff = analyzer.spectral_rolloff(&spectrum, 0.85);
        assert!(rolloff > 0.0);
        
        // Test zero crossing rate
        let zcr = analyzer.zero_crossing_rate(&samples);
        assert!(zcr > 0.0);
    }
    
    #[test]
    fn test_formant_extraction() {
        let analyzer = SpectrumAnalyzer::new_for_speech();
        let sample_rate = 44100;
        
        // Generate a complex signal with multiple frequencies
        let mut samples = generate_sine_wave(500.0, sample_rate, 0.5);
        let samples2 = generate_sine_wave(1500.0, sample_rate, 0.5);
        let samples3 = generate_sine_wave(2500.0, sample_rate, 0.5);
        
        for (i, (s2, s3)) in samples2.iter().zip(samples3.iter()).enumerate() {
            samples[i] += s2 + s3;
        }
        
        let spectrum = analyzer.analyze(&samples, sample_rate).unwrap();
        let formants = analyzer.extract_formants(&spectrum, 3);
        
        assert!(!formants.is_empty());
        assert!(formants.len() <= 3);
    }
}