import { User } from "../_model/pagination";

export const PhotoHelper = {
    parseUser: function (user: User): User {
        // Add default avatar if not present
        if (!user.avatar_url) {
            user.avatar_url = '/assets/default-avatar.png';
        }
        return user;
    }
};
