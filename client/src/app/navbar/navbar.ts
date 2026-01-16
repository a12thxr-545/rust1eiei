import { Component, computed, inject, signal, Signal } from '@angular/core';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatMenuModule } from '@angular/material/menu';
import { NavigationEnd, Router, RouterLink } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { filter } from 'rxjs';

@Component({
  selector: 'app-navbar',
  imports: [MatToolbarModule, MatButtonModule, MatIconModule, MatMenuModule, RouterLink],
  templateUrl: './navbar.html',
  styleUrl: './navbar.css',
})
export class Navbar {
  private _passport = inject(PassportService);
  private _router = inject(Router);

  display_name: Signal<string | undefined>
  avatar_url: Signal<string | undefined>
  isLoggedIn: Signal<boolean>
  isOnLoginPage = signal<boolean>(false);

  constructor() {
    this.display_name = computed(() => this._passport.data()?.display_name);
    this.avatar_url = computed(() => this._passport.data()?.avatar_url || '/assets/defaultavtar.jpg');
    this.isLoggedIn = computed(() => this._passport.data() !== undefined);

    // Check initial route
    this.isOnLoginPage.set(this._router.url === '/login');

    // Listen for route changes
    this._router.events.pipe(
      filter(event => event instanceof NavigationEnd)
    ).subscribe((event: NavigationEnd) => {
      this.isOnLoginPage.set(event.urlAfterRedirects === '/login');
    });
  }

  logout() {
    this._passport.removePassport();
    this._router.navigate(['/login']);
  }

  goToProfile() {
    this._router.navigate(['/profile']);
  }
}
