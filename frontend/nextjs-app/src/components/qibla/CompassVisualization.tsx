'use client';

import { useEffect, useRef } from 'react';
import { CompassState } from '@/types/qibla';
import { QiblaService } from '@/lib/services/qibla-service';

interface CompassVisualizationProps {
  compassState: CompassState;
  isNightMode?: boolean;
}

export default function CompassVisualization({
  compassState,
  isNightMode = false,
}: CompassVisualizationProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationRef = useRef<number>();

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const drawCompass = () => {
      const width = canvas.width;
      const height = canvas.height;
      const centerX = width / 2;
      const centerY = height / 2;
      const radius = Math.min(width, height) / 2 - 40;

      // Clear canvas
      ctx.clearRect(0, 0, width, height);

      // Draw background gradient
      const gradient = ctx.createRadialGradient(
        centerX,
        centerY,
        0,
        centerX,
        centerY,
        radius
      );
      if (isNightMode) {
        gradient.addColorStop(0, 'rgba(27, 54, 93, 0.3)');
        gradient.addColorStop(1, 'rgba(15, 31, 53, 0.1)');
      } else {
        gradient.addColorStop(0, 'rgba(184, 134, 11, 0.1)');
        gradient.addColorStop(1, 'rgba(255, 255, 255, 0)');
      }
      ctx.fillStyle = gradient;
      ctx.fillRect(0, 0, width, height);

      // Save context for rotation
      ctx.save();
      ctx.translate(centerX, centerY);
      ctx.rotate((-compassState.heading * Math.PI) / 180);

      // Draw compass rose
      drawCompassRose(ctx, radius, isNightMode);

      ctx.restore();

      // Draw Qibla indicator (fixed position, rotated by relative direction)
      ctx.save();
      ctx.translate(centerX, centerY);
      ctx.rotate((compassState.relativeDirection * Math.PI) / 180);

      const isPointingToQibla = QiblaService.isPointingToQibla(
        compassState.relativeDirection
      );
      drawQiblaIndicator(ctx, radius, isPointingToQibla, isNightMode);

      ctx.restore();

      // Draw center icon
      drawCenterIcon(ctx, centerX, centerY, isNightMode);

      // Draw heading text
      drawHeadingText(
        ctx,
        centerX,
        40,
        compassState.heading,
        isNightMode
      );

      // Draw direction indicator
      drawDirectionIndicator(
        ctx,
        centerX,
        height - 60,
        compassState.relativeDirection,
        isPointingToQibla,
        isNightMode
      );
    };

    drawCompass();
    animationRef.current = requestAnimationFrame(drawCompass);

    return () => {
      if (animationRef.current) {
        cancelAnimationFrame(animationRef.current);
      }
    };
  }, [compassState, isNightMode]);

  // Resize canvas to match container
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const resizeCanvas = () => {
      const container = canvas.parentElement;
      if (container) {
        canvas.width = container.clientWidth;
        canvas.height = 500;
      }
    };

    resizeCanvas();
    window.addEventListener('resize', resizeCanvas);

    return () => {
      window.removeEventListener('resize', resizeCanvas);
    };
  }, []);

  return (
    <div
      className={`rounded-2xl shadow-2xl overflow-hidden ${
        isNightMode ? 'bg-[#1B365D]/50' : 'bg-white'
      }`}
    >
      <canvas ref={canvasRef} className="w-full" />
    </div>
  );
}

function drawCompassRose(
  ctx: CanvasRenderingContext2D,
  radius: number,
  isNightMode: boolean
) {
  const color = isNightMode ? '#B8860B' : '#1B365D';

  // Draw outer circle
  ctx.strokeStyle = color + '4D'; // 30% opacity
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.arc(0, 0, radius - 10, 0, 2 * Math.PI);
  ctx.stroke();

  // Draw cardinal directions
  const directions = ['ش', 'ق', 'ج', 'غ']; // N, E, S, W in Arabic
  const angles = [0, 90, 180, 270];

  ctx.font = 'bold 24px Tajawal, sans-serif';
  ctx.fillStyle = isNightMode ? '#FFFFFF' : color;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';

  directions.forEach((dir, i) => {
    const angle = (angles[i] * Math.PI) / 180;
    const x = (radius - 40) * Math.sin(angle);
    const y = -(radius - 40) * Math.cos(angle);
    ctx.fillText(dir, x, y);
  });

  // Draw tick marks
  ctx.strokeStyle = color + '80'; // 50% opacity
  ctx.lineWidth = 2;

  for (let i = 0; i < 360; i += 10) {
    const angle = (i * Math.PI) / 180;
    const isMajor = i % 30 === 0;
    const tickLength = isMajor ? 15 : 8;

    const startX = (radius - 10) * Math.sin(angle);
    const startY = -(radius - 10) * Math.cos(angle);
    const endX = (radius - 10 - tickLength) * Math.sin(angle);
    const endY = -(radius - 10 - tickLength) * Math.cos(angle);

    ctx.beginPath();
    ctx.moveTo(startX, startY);
    ctx.lineTo(endX, endY);
    ctx.stroke();
  }
}

function drawQiblaIndicator(
  ctx: CanvasRenderingContext2D,
  radius: number,
  isPointingToQibla: boolean,
  isNightMode: boolean
) {
  const color = isPointingToQibla
    ? isNightMode
      ? '#4ADE80'
      : '#22C55E'
    : isNightMode
    ? '#B8860B'
    : '#1B365D';

  // Draw arrow
  ctx.fillStyle = color;
  ctx.beginPath();
  ctx.moveTo(0, -radius + 60);
  ctx.lineTo(-20, -radius + 100);
  ctx.lineTo(0, -radius + 90);
  ctx.lineTo(20, -radius + 100);
  ctx.closePath();
  ctx.fill();

  // Draw Kaaba label
  ctx.font = 'bold 16px Tajawal, sans-serif';
  ctx.fillStyle = isNightMode ? '#FFFFFF' : '#1A1A1A';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';

  const labelY = -radius + 120;
  const padding = 12;

  // Background
  ctx.fillStyle = color + '33'; // 20% opacity
  const textMetrics = ctx.measureText('الكعبة');
  ctx.fillRect(
    -textMetrics.width / 2 - padding,
    labelY - 10,
    textMetrics.width + padding * 2,
    24
  );

  // Text
  ctx.fillStyle = isNightMode ? '#FFFFFF' : '#1A1A1A';
  ctx.fillText('الكعبة', 0, labelY);
}

function drawCenterIcon(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  isNightMode: boolean
) {
  const color = isNightMode ? '#B8860B' : '#1B365D';

  // Draw circle
  ctx.strokeStyle = color;
  ctx.lineWidth = 3;
  ctx.fillStyle = isNightMode ? 'rgba(27, 54, 93, 0.5)' : '#FFFFFF';
  ctx.beginPath();
  ctx.arc(x, y, 40, 0, 2 * Math.PI);
  ctx.fill();
  ctx.stroke();

  // Draw mosque icon (simplified)
  ctx.fillStyle = color;
  ctx.font = '32px Arial';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText('🕌', x, y);
}

function drawHeadingText(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  heading: number,
  isNightMode: boolean
) {
  const direction = QiblaService.getCardinalDirection(heading);
  const headingText = `${Math.round(heading)}°`;

  // Background
  ctx.fillStyle = isNightMode ? 'rgba(27, 54, 93, 0.8)' : '#FFFFFF';
  ctx.strokeStyle = isNightMode ? 'rgba(184, 134, 11, 0.5)' : 'rgba(27, 54, 93, 0.3)';
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.roundRect(x - 80, y - 20, 160, 40, 20);
  ctx.fill();
  ctx.stroke();

  // Heading number
  ctx.font = 'bold 24px Tajawal, sans-serif';
  ctx.fillStyle = isNightMode ? '#B8860B' : '#1B365D';
  ctx.textAlign = 'right';
  ctx.textBaseline = 'middle';
  ctx.fillText(headingText, x + 20, y);

  // Direction text
  ctx.font = '18px Tajawal, sans-serif';
  ctx.fillStyle = isNightMode ? 'rgba(255, 255, 255, 0.7)' : '#666666';
  ctx.textAlign = 'left';
  ctx.fillText(direction, x + 30, y);
}

function drawDirectionIndicator(
  ctx: CanvasRenderingContext2D,
  x: number,
  y: number,
  relativeDirection: number,
  isPointingToQibla: boolean,
  isNightMode: boolean
) {
  let text: string;
  let icon: string;

  if (isPointingToQibla) {
    text = 'أنت تتجه نحو القبلة ✓';
    icon = '✓';
  } else if (relativeDirection > 0) {
    text = `اتجه يميناً ${Math.abs(Math.round(relativeDirection))}°`;
    icon = '→';
  } else {
    text = `اتجه يساراً ${Math.abs(Math.round(relativeDirection))}°`;
    icon = '←';
  }

  const color = isPointingToQibla
    ? isNightMode
      ? '#4ADE80'
      : '#22C55E'
    : isNightMode
    ? '#B8860B'
    : '#1B365D';

  // Background
  ctx.fillStyle = isNightMode ? 'rgba(27, 54, 93, 0.3)' : '#F8F9FA';
  ctx.beginPath();
  ctx.roundRect(x - 180, y - 25, 360, 50, 12);
  ctx.fill();

  // Text
  ctx.font = 'bold 20px Tajawal, sans-serif';
  ctx.fillStyle = color;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText(text, x, y);
}
