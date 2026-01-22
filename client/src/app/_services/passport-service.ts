import { HttpClient } from "@angular/common/http"
import { environment } from "../../environments/environment.development"
import { inject, Injectable, PLATFORM_ID, signal } from "@angular/core"
import { LoginModel, Passport, RegisterBrawlerModel } from "../_model/passport"
import { firstValueFrom } from "rxjs"
import { isPlatformBrowser } from "@angular/common"

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


    data = signal<undefined | Passport>(undefined)


    private get isBrowser(): boolean {
        return isPlatformBrowser(this._platformId)
    }

    private loadPassportFormLocalStorage(): string | null {
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
        this.loadPassportFormLocalStorage()
    }

    async get(login: LoginModel): Promise<null | string> {
        try {
            const api_url = this._base_url + '/authentication/login'
            await this.fetchPassport(api_url, login)
        } catch (error) {
            return ` ${error}`
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
            return ` ${error}`
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
}