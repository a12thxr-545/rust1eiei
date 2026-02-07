import { Routes } from "@angular/router";
import { Home } from "./home/home";
import { Login } from "./login/login";
import { Profile } from "./profile/profile";
import { ProfileDetail } from "./profile/profile-detail/profile-detail";
import { NotFound } from "./not-found/not-found";
import { ServerError } from "./server-error/server-error";
import { MissionComponent } from "./mission/mission";
import { authGuard } from "./auth-guard";
export const routes: Routes = [
    { path: '', component: Home, },
    { path: 'login', component: Login, },
    { path: 'missions', component: MissionComponent, canActivate: [authGuard] },
    { path: 'profile', component: Profile, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    { path: 'u/:username', component: ProfileDetail, canActivate: [authGuard] },
    { path: 'server-error', component: ServerError, },
    { path: '**', component: NotFound, },
];