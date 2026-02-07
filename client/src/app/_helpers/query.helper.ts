export function parseQuery<T extends { [key: string]: any }>(query: T): string {
    const params = new URLSearchParams();

    for (const [key, value] of Object.entries(query)) {
        if (value !== undefined && value !== null && value !== '') {
            params.append(key, String(value));
        }
    }

    const queryString = params.toString();
    return queryString ? `?${queryString}` : '';
}
