export function formatRelativeTime(isoString: string): string {
    const then = new Date(isoString).getTime();
    const now = Date.now();
    const diffSecs = Math.floor((now - then) / 1000);
    
    if (diffSecs < 60) return `${Math.max(0, diffSecs)} sec ago`;
    
    const diffMins = Math.floor(diffSecs / 60);
    if (diffMins < 60) return `${diffMins} min ago`;
    
    const diffHrs = Math.floor(diffMins / 60);
    if (diffHrs < 24) return `${diffHrs} hours ago`;
    
    return `${Math.floor(diffHrs / 24)} days ago`;
}
