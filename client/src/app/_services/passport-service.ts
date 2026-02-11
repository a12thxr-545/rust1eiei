import { HttpClient, HttpErrorResponse } from "@angular/common/http"
import { environment } from "../../environments/environment.development"
import { inject, Injectable, PLATFORM_ID, signal } from "@angular/core"
import { LoginModel, Passport, RegisterBrawlerModel } from "../_model/passport"
import { firstValueFrom } from "rxjs"
import { isPlatformBrowser } from "@angular/common"
import { Router } from "@angular/router"

export interface UploadedImage {
    url: string;
    public_id: string;
}

@Injectable({
    providedIn: 'root'
})

export class PassportService {
    private _key = 'passport'
    private _base_url = environment.base_url + '/api'
    private _http = inject(HttpClient)
    private _platformId = inject(PLATFORM_ID)
    private _router = inject(Router)


    data = signal<undefined | Passport>(undefined)


    private get isBrowser(): boolean {
        return isPlatformBrowser(this._platformId)
    }

    public loadPassportFromLocalStorage(): string | null {
        if (!this.isBrowser) return null
        const jsonString = localStorage.getItem(this._key)
        if (!jsonString) return 'notfound'
        try {
            const passport = JSON.parse(jsonString) as Passport
            this.data.set(passport)
        } catch (error) {
            return ` ${error}`
        }
        return null
    }

    private savePassportToLocalStorage() {
        if (!this.isBrowser) return
        const passport = this.data()
        if (!passport) return
        localStorage.setItem(this._key, JSON.stringify(passport))
    }

    removePassport() {
        if (!this.isBrowser) return
        localStorage.removeItem(this._key)
        this.data.set(undefined)
    }

    constructor() {
        this.loadPassportFromLocalStorage()
    }

    async get(login: LoginModel): Promise<null | string> {
        try {
            const api_url = this._base_url + '/authentication/login'
            await this.fetchPassport(api_url, login)
        } catch (error) {
            if (error instanceof HttpErrorResponse) {
                return error.error || error.message;
            }
            return `${error}`;
        }

        return null
    }

    private async fetchPassport(api_url: string, model: LoginModel | RegisterBrawlerModel) {
        const result = this._http.post<Passport>(api_url, model)
        const passport = await firstValueFrom(result)
        this.data.set(passport)
        this.savePassportToLocalStorage()
    }

    async register(model: RegisterBrawlerModel): Promise<null | string> {
        try {
            const api_url = this._base_url + '/brawlers/register'
            await this.fetchPassport(api_url, model)
        } catch (error) {
            if (error instanceof HttpErrorResponse) {
                return error.error || error.message;
            }
            return `${error}`;
        }
        return null
    }

    async uploadAvatar(base64String: string): Promise<UploadedImage | null> {
        try {
            const api_url = this._base_url + '/brawlers/avatar'
            const result = this._http.post<UploadedImage>(api_url, { base64_string: base64String })
            const uploadedImage = await firstValueFrom(result)

            // Update passport with new avatar URL
            this.updateAvatarUrl(uploadedImage.url)

            return uploadedImage
        } catch (error) {
            console.error('Upload avatar error:', error)
            return null
        }
    }

    updateAvatarUrl(url: string) {
        const passport = this.data()
        if (!passport) return

        const updatedPassport = { ...passport, avatar_url: url }
        this.data.set(updatedPassport)
        this.savePassportToLocalStorage()
    }

    async uploadCover(base64String: string): Promise<UploadedImage | null> {
        try {
            const api_url = this._base_url + '/brawlers/cover'
            const result = this._http.post<UploadedImage>(api_url, { base64_string: base64String })
            const uploadedImage = await firstValueFrom(result)

            // Update passport with new cover URL
            this.updateCoverUrl(uploadedImage.url)

            return uploadedImage
        } catch (error) {
            console.error('Upload cover error:', error)
            return null
        }
    }

    updateCoverUrl(url: string) {
        const passport = this.data()
        if (!passport) return

        const updatedPassport = { ...passport, cover_url: url }
        this.data.set(updatedPassport)
        this.savePassportToLocalStorage()
    }

    async syncProfile(): Promise<void> {
        if (!this.isBrowser) return

        const passport = this.data()
        if (!passport) return

        try {
            const api_url = this._base_url + '/brawlers/profile'
            const result = this._http.get<Passport>(api_url)
            const freshPassport = await firstValueFrom(result)

            // Update local data with fresh data from server
            this.data.set(freshPassport)
            this.savePassportToLocalStorage()
        } catch (error) {
            console.error('Failed to sync profile:', error)
            if (error instanceof HttpErrorResponse && error.status === 401) {
                this.removePassport();
                this._router.navigate(['/login']);
            }
        }
    }

    async checkUsername(username: string): Promise<boolean> {
        try {
            const api_url = `${this._base_url}/brawlers/check-username/${username}`
            const result = this._http.get<boolean>(api_url)
            return await firstValueFrom(result)
        } catch (error) {
            console.error('Check username error:', error)
            return false
        }
    }

    async updateDisplayName(displayName: string): Promise<string | null> {
        try {
            const api_url = `${this._base_url}/brawlers/display-name`
            const result = this._http.put<Passport>(api_url, { displayName })
            const updatedPassport = await firstValueFrom(result)

            // Update local data
            this.data.set(updatedPassport)
            this.savePassportToLocalStorage()

            return null // Success
        } catch (error) {
            console.error('Update display name error:', error)
            if (error instanceof HttpErrorResponse) {
                return error.error || error.message
            }
            return `${error}`
        }
    }

    async uploadChatImage(base64String: string): Promise<UploadedImage | null> {
        try {
            const api_url = this._base_url + '/brawlers/chat-image'
            const result = this._http.post<UploadedImage>(api_url, { base64_string: base64String })
            return await firstValueFrom(result)
        } catch (error) {
            console.error('Upload chat image error:', error)
            return null
        }
    }

    async updateBio(bio: string): Promise<boolean> {
        try {
            const api_url = `${this._base_url}/brawlers/bio`
            const result = this._http.put<any>(api_url, { bio })
            const updatedProfile = await firstValueFrom(result)

            // Update local passport with new bio
            const passport = this.data()
            if (passport) {
                const updatedPassport = { ...passport, bio: updatedProfile.bio }
                this.data.set(updatedPassport)
                this.savePassportToLocalStorage()
            }

            return true
        } catch (error) {
            console.error('Update bio error:', error)
            return false
        }
    }
}