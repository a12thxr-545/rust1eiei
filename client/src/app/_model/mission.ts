export interface Mission {
    id: number;
    name: string;
    description?: string;
    status: string;
    chief_id: number;
    crew_count: number;
    created_at: string;
    updated_at: string;
}

export interface AddMission {
    name: string;
    description?: string;
}

export interface MissionFilter {
    name?: string;
    status?: string;
}
