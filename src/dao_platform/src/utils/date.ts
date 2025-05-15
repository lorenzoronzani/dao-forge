export const formatDate = (timestamp: number) => {
    return new Date(timestamp).toLocaleDateString('en-IT', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
    });
};