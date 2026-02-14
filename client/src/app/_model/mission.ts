export interface UploadedImage {
    url: string;
    public_id: string;
}

export interface Mission {
    id: number;
    name: string;
    description?: string;
    status: string;
    chief_id: number;
    chief_name: string;
    crew_count: number;
    image_url?: string;
    code: string;
    max_participants: number;
    created_at: string;
    updated_at: string;
}

export interface AddMission {
    name: string;
    description?: string;
    image_url?: string;
    max_participants: number;
}

export interface EditMission {
    name?: string;
    description?: string;
    max_participants?: number;
}

export interface MissionFilter {
    name?: string;
    code?: string;
    status?: string;
    chief_id?: number;
    exclude_chief_id?: number;
    member_id?: number;
    exclude_member_id?: number;
}

export interface CrewMember {
    brawler_id: number;
    display_name: string;
    username: string;
    avatar_url: string;
    mission_success_count: number;
    mission_joined_count: number;
}
