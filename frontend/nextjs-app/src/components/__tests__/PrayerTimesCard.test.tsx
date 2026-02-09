import React from 'react';
import { render, screen, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import { PrayerTimesCard } from '../dashboard/PrayerTimesCard';

/**
 * Component tests for Prayer Times Card
 * **Validates: Requirements 20.2**
 */
describe('PrayerTimesCard Component', () => {
  const mockPrayerTimes = {
    fajr: '05:30',
    sunrise: '06:45',
    dhuhr: '12:30',
    asr: '15:45',
    maghrib: '18:15',
    isha: '19:30',
    date: new Date('2024-01-15'),
  };

  it('should render all prayer times', () => {
    // Act
    render(<PrayerTimesCard prayerTimes={mockPrayerTimes} />);

    // Assert
    expect(screen.getByText('الفجر')).toBeInTheDocument();
    expect(screen.getByText('05:30')).toBeInTheDocument();
    expect(screen.getByText('الظهر')).toBeInTheDocument();
    expect(screen.getByText('12:30')).toBeInTheDocument();
    expect(screen.getByText('المغرب')).toBeInTheDocument();
    expect(screen.getByText('18:15')).toBeInTheDocument();
  });

  it('should highlight next prayer', () => {
    // Arrange
    const currentTime = new Date('2024-01-15T14:00:00');

    // Act
    render(
      <PrayerTimesCard
        prayerTimes={mockPrayerTimes}
        currentTime={currentTime}
      />
    );

    // Assert
    const asrElement = screen.getByText('العصر').closest('div');
    expect(asrElement).toHaveClass('highlighted');
  });

  it('should show countdown to next prayer', () => {
    // Arrange
    const currentTime = new Date('2024-01-15T14:00:00');

    // Act
    render(
      <PrayerTimesCard
        prayerTimes={mockPrayerTimes}
        currentTime={currentTime}
      />
    );

    // Assert
    expect(screen.getByText(/باقي/)).toBeInTheDocument();
    expect(screen.getByText(/1:45/)).toBeInTheDocument();
  });

  it('should display loading state', () => {
    // Act
    render(<PrayerTimesCard loading={true} />);

    // Assert
    expect(screen.getByRole('progressbar')).toBeInTheDocument();
  });

  it('should display error message', () => {
    // Arrange
    const errorMessage = 'فشل في تحميل المواقيت';

    // Act
    render(<PrayerTimesCard error={errorMessage} />);

    // Assert
    expect(screen.getByText(errorMessage)).toBeInTheDocument();
  });

  it('should show location information', () => {
    // Arrange
    const location = {
      city: 'الرياض',
      country: 'السعودية',
    };

    // Act
    render(
      <PrayerTimesCard
        prayerTimes={mockPrayerTimes}
        location={location}
      />
    );

    // Assert
    expect(screen.getByText('الرياض، السعودية')).toBeInTheDocument();
  });

  it('should update countdown every second', async () => {
    // Arrange
    jest.useFakeTimers();
    const currentTime = new Date('2024-01-15T14:00:00');

    // Act
    render(
      <PrayerTimesCard
        prayerTimes={mockPrayerTimes}
        currentTime={currentTime}
      />
    );

    // Fast-forward 1 second
    jest.advanceTimersByTime(1000);

    // Assert
    await waitFor(() => {
      expect(screen.getByText(/1:44:59/)).toBeInTheDocument();
    });

    jest.useRealTimers();
  });

  it('should handle missing prayer times gracefully', () => {
    // Act
    render(<PrayerTimesCard prayerTimes={null} />);

    // Assert
    expect(screen.getByText(/لا توجد مواقيت متاحة/)).toBeInTheDocument();
  });

  it('should apply RTL direction for Arabic text', () => {
    // Act
    const { container } = render(
      <PrayerTimesCard prayerTimes={mockPrayerTimes} />
    );

    // Assert
    const card = container.firstChild as HTMLElement;
    expect(card).toHaveAttribute('dir', 'rtl');
  });

  it('should show sunrise time separately', () => {
    // Act
    render(<PrayerTimesCard prayerTimes={mockPrayerTimes} />);

    // Assert
    expect(screen.getByText('الشروق')).toBeInTheDocument();
    expect(screen.getByText('06:45')).toBeInTheDocument();
  });

  it('should format times in 12-hour format when specified', () => {
    // Act
    render(
      <PrayerTimesCard
        prayerTimes={mockPrayerTimes}
        timeFormat="12h"
      />
    );

    // Assert
    expect(screen.getByText('5:30 AM')).toBeInTheDocument();
    expect(screen.getByText('6:15 PM')).toBeInTheDocument();
  });

  it('should show notification icon for enabled prayers', () => {
    // Arrange
    const notificationsEnabled = ['fajr', 'dhuhr', 'asr', 'maghrib', 'isha'];

    // Act
    render(
      <PrayerTimesCard
        prayerTimes={mockPrayerTimes}
        notificationsEnabled={notificationsEnabled}
      />
    );

    // Assert
    const notificationIcons = screen.getAllByTestId('notification-icon');
    expect(notificationIcons).toHaveLength(5);
  });

  it('should be accessible with screen readers', () => {
    // Act
    render(<PrayerTimesCard prayerTimes={mockPrayerTimes} />);

    // Assert
    expect(screen.getByRole('region')).toHaveAttribute(
      'aria-label',
      'مواقيت الصلاة'
    );
  });

  it('should handle different madhabs correctly', () => {
    // Arrange
    const hanafiFajr = { ...mockPrayerTimes, fajr: '05:45' };

    // Act
    render(
      <PrayerTimesCard
        prayerTimes={hanafiFajr}
        madhab="hanafi"
      />
    );

    // Assert
    expect(screen.getByText('05:45')).toBeInTheDocument();
  });
});
