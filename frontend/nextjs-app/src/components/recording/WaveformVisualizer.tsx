'use client';

import React, { useEffect, useRef } from 'react';
import { WaveformData } from '@/types/recording';

interface WaveformVisualizerProps {
  waveformData?: WaveformData;
  waveColor?: string;
  backgroundColor?: string;
  height?: number;
  showGrid?: boolean;
}

export const WaveformVisualizer: React.FC<WaveformVisualizerProps> = ({
  waveformData,
  waveColor = '#B8860B', // Gold
  backgroundColor = '#1B365D', // Navy
  height = 120,
  showGrid = true,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
    ctx.scale(dpr, dpr);

    // Clear canvas
    ctx.clearRect(0, 0, rect.width, rect.height);

    // Draw grid
    if (showGrid) {
      drawGrid(ctx, rect.width, rect.height);
    }

    // Draw waveform or placeholder
    if (waveformData && waveformData.amplitudes.length > 0) {
      drawWaveform(ctx, rect.width, rect.height, waveformData, waveColor);
    } else {
      drawPlaceholder(ctx, rect.width, rect.height);
    }
  }, [waveformData, waveColor, showGrid]);

  const drawGrid = (
    ctx: CanvasRenderingContext2D,
    width: number,
    height: number
  ) => {
    ctx.strokeStyle = 'rgba(128, 128, 128, 0.2)';
    ctx.lineWidth = 0.5;

    // Horizontal lines
    for (let i = 0; i <= 4; i++) {
      const y = (height * i) / 4;
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(width, y);
      ctx.stroke();
    }

    // Vertical lines
    for (let i = 0; i <= 10; i++) {
      const x = (width * i) / 10;
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, height);
      ctx.stroke();
    }
  };

  const drawWaveform = (
    ctx: CanvasRenderingContext2D,
    width: number,
    height: number,
    data: WaveformData,
    color: string
  ) => {
    const amplitudes = data.amplitudes;
    const centerY = height / 2;
    const maxVisibleBars = 100;

    // Get visible amplitudes
    const startIndex = Math.max(0, amplitudes.length - maxVisibleBars);
    const visibleAmplitudes = amplitudes.slice(startIndex);

    // Calculate bar dimensions
    const barWidth = width / maxVisibleBars;
    const barSpacing = barWidth * 0.3;
    const actualBarWidth = barWidth - barSpacing;

    ctx.fillStyle = color;

    // Draw bars
    visibleAmplitudes.forEach((amplitude, i) => {
      const x = i * barWidth + barSpacing / 2;
      const barHeight = amplitude * (height / 2) * 0.8;

      // Draw rounded rectangle
      const rectX = x;
      const rectY = centerY - barHeight / 2;
      const rectWidth = actualBarWidth;
      const rectHeight = barHeight;
      const radius = 2;

      ctx.beginPath();
      ctx.moveTo(rectX + radius, rectY);
      ctx.lineTo(rectX + rectWidth - radius, rectY);
      ctx.quadraticCurveTo(rectX + rectWidth, rectY, rectX + rectWidth, rectY + radius);
      ctx.lineTo(rectX + rectWidth, rectY + rectHeight - radius);
      ctx.quadraticCurveTo(
        rectX + rectWidth,
        rectY + rectHeight,
        rectX + rectWidth - radius,
        rectY + rectHeight
      );
      ctx.lineTo(rectX + radius, rectY + rectHeight);
      ctx.quadraticCurveTo(rectX, rectY + rectHeight, rectX, rectY + rectHeight - radius);
      ctx.lineTo(rectX, rectY + radius);
      ctx.quadraticCurveTo(rectX, rectY, rectX + radius, rectY);
      ctx.closePath();
      ctx.fill();
    });
  };

  const drawPlaceholder = (
    ctx: CanvasRenderingContext2D,
    width: number,
    height: number
  ) => {
    const centerY = height / 2;

    // Draw flat line
    ctx.strokeStyle = 'rgba(128, 128, 128, 0.3)';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(0, centerY);
    ctx.lineTo(width, centerY);
    ctx.stroke();

    // Draw text
    ctx.fillStyle = 'rgba(128, 128, 128, 0.6)';
    ctx.font = '14px Tajawal, sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText('ابدأ التسجيل لرؤية الموجات الصوتية', width / 2, centerY);
  };

  return (
    <div
      className="rounded-xl border overflow-hidden"
      style={{
        backgroundColor: `${backgroundColor}1A`, // 10% opacity
        borderColor: `${backgroundColor}4D`, // 30% opacity
        height: `${height}px`,
      }}
    >
      <canvas
        ref={canvasRef}
        className="w-full h-full"
        style={{ height: `${height}px` }}
      />
    </div>
  );
};

interface AnimatedWaveformBarsProps {
  isRecording: boolean;
  color?: string;
  height?: number;
}

export const AnimatedWaveformBars: React.FC<AnimatedWaveformBarsProps> = ({
  isRecording,
  color = '#B8860B',
  height = 40,
}) => {
  if (!isRecording) {
    return <div style={{ height: `${height}px` }} />;
  }

  return (
    <div className="flex items-center justify-center gap-1" style={{ height: `${height}px` }}>
      {[0, 1, 2, 3, 4].map((index) => (
        <div
          key={index}
          className="w-1 rounded-full animate-pulse"
          style={{
            backgroundColor: color,
            animationDelay: `${index * 0.2}s`,
            animationDuration: '0.8s',
            height: '100%',
          }}
        />
      ))}
    </div>
  );
};
