const ONE_DAY = 1000 * 60 * 60 * 24;
const ONE_HOUR = 1000 * 60 * 60;
const ONE_MINUTE = 1000 * 60;

export const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString('en-IT', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
    });
};

export const getTimeRemaining = (endDate: Date): string => {
    const timeLeft = endDate.getTime() - Date.now();

    if (timeLeft <= 0) return 'Ended';

    const days = Math.floor(timeLeft / ONE_DAY);
    const hours = Math.floor((timeLeft % ONE_DAY) / ONE_HOUR);
    const minutes = Math.floor((timeLeft % ONE_HOUR) / ONE_MINUTE);

    if (days > 0) return `${days}d ${hours}h remaining`;
    if (hours > 0) return `${hours}h ${minutes}m remaining`;
    return `${minutes}m remaining`;
};
