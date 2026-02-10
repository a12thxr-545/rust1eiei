export interface Friend {
    friendship_id: number;
    friend_id: number;
    display_name: string;
    username: string;
    avatar_url: string | null;
    status: string;
}

export interface MissionInvitation {
    invitation_id: number;
    mission_id: number;
    mission_name: string;
    inviter_id: number;
    inviter_name: string;
    invitee_id: number;
    invitee_name: string;
    status: string;
    created_at: string;
}
