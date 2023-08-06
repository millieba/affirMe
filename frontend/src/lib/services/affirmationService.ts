export interface Affirmation {
    id: number;
    text: string;
}

export async function fetchRandomAffirmation(): Promise<Affirmation> {
    const response = await fetch('http://localhost:8080/affirmations/random');
    if (!response.ok) {
        throw new Error('Failed to fetch affirmation');
    }
    return response.json();
}
