import { Component, inject, OnInit, PLATFORM_ID } from '@angular/core';
import { RouterOutlet, Router, NavigationEnd } from '@angular/router';
import { Navbar } from "./navbar/navbar";
import { PassportService } from './_services/passport-service';
import { isPlatformBrowser } from '@angular/common';
import { filter } from 'rxjs';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, Navbar],
  templateUrl: './app.html',
  styleUrl: './app.css'
})
export class App implements OnInit {
  private passportService = inject(PassportService);
  private router = inject(Router);
  private platformId = inject(PLATFORM_ID);

  ngOnInit(): void {
    if (isPlatformBrowser(this.platformId)) {
      // Sync on startup
      this.passportService.syncProfile();

      // Sync on navigation to keep global state fresh
      this.router.events.pipe(
        filter(event => event instanceof NavigationEnd)
      ).subscribe(() => {
        this.passportService.syncProfile();
      });
    }
  }
}
