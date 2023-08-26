export interface Affirmation {
    text: string;
    tags: string[];
}
export interface PaginatedAffirmations {
    affirmations: Affirmation[];
    total_documents: number;
}


export async function fetchRandomAffirmation(): Promise<Affirmation> {
    const response = await fetch('http://localhost:8080/affirmations/random');
    if (!response.ok) {
        throw new Error('Failed to fetch affirmation');
    }
    return response.json();
}

export async function fetchAllAffirmations(pageNumber: number, itemsPerPage: number): Promise<PaginatedAffirmations> {
    const response = await fetch(`http://localhost:8080/affirmations?page_number=${pageNumber}&items_per_page=${itemsPerPage}`);
    if (!response.ok) {
        throw new Error('Failed to fetch affirmation');
    }
    return response.json();
}

export async function fetchDropdownOptions(): Promise<string[]> {
    const response = await fetch('http://localhost:8080/affirmations/tags');
    if (!response.ok) {
        throw new Error('Failed to fetch affirmation');
    }
    return response.json();
}

export async function fetchAffirmations(searchInput: string, pageNumber: number, itemsPerPage: number, tags?: string[]): Promise<PaginatedAffirmations> {
    // if tags are provided, parse them to a comma separated string
    let tagString = '';
    if (tags) {
        tagString = tags.join(',');
    }
    const response = await fetch(`http://localhost:8080/affirmations?search=${searchInput}&page_number=${pageNumber}&items_per_page=${itemsPerPage}&tags=${tagString}`);
    if (!response.ok) {
        throw new Error('Failed to fetch affirmation');
    }
    return response.json();
}
