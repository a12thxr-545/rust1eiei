import { Component, inject, OnInit, signal } from '@angular/core';
import { Router, NavigationEnd } from '@angular/router';
import { filter } from 'rxjs';
import { isPlatformBrowser } from '@angular/common';
import { PLATFORM_ID } from '@angular/core';
import { Mission } from '../_model/mission';
import { MissionService } from '../_services/mission-service';
import { PassportService } from '../_services/passport-service';
import { DatePipe } from '@angular/common';

@Component({
  selector: 'app-home',
  imports: [DatePipe],
  templateUrl: './home.html',
  styleUrl: './home.css',
})
export class Home implements OnInit {
  private missionService = inject(MissionService);
  private passportService = inject(PassportService);
  private router = inject(Router);
  private platformId = inject(PLATFORM_ID);

  missions = signal<Mission[]>([]);
  isLoggedIn = this.passportService.data;

  stats = {
    totalMissions: 0,
    activeMissions: 0,
    totalCrew: 0
  };

  features = [
    {
      title: 'Create Missions',
      description: 'Launch your own missions and lead your crew.'
    },
    {
      title: 'Build Crew',
      description: 'Join missions or build your own elite team.'
    },
    {
      title: 'Explore',
      description: 'Collaborate with others to complete missions.'
    },
    {
      title: 'Earn Rewards',
      description: 'Complete missions and climb the ranks.'
    }
  ];

  async ngOnInit() {
    if (isPlatformBrowser(this.platformId)) {
      await this.loadMissions();

      this.router.events.pipe(
        filter(event => event instanceof NavigationEnd)
      ).subscribe(() => {
        this.loadMissions();
      });
    }
  }

  async loadMissions() {
    const missions = await this.missionService.getMissions();
    this.missions.set(missions.slice(0, 3));

    this.stats.totalMissions = missions.length;
    this.stats.activeMissions = missions.filter(m => m.status.toLowerCase() === 'open').length;
    this.stats.totalCrew = missions.reduce((sum, m) => sum + m.crew_count, 0);
  }

  navigateToMissions() {
    this.router.navigate(['/missions']);
  }

  navigateToLogin() {
    this.router.navigate(['/login']);
  }
}
