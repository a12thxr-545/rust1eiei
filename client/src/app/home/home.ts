import { Component, inject, OnInit, signal } from '@angular/core';
import { Mission } from '../_model/mission';
import { MissionService } from '../_services/mission-service';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatChipsModule } from '@angular/material/chips';
import { DatePipe } from '@angular/common';

@Component({
  selector: 'app-home',
  imports: [MatCardModule, MatButtonModule, MatIconModule, MatChipsModule, DatePipe],
  templateUrl: './home.html',
  styleUrl: './home.css',
})
export class Home implements OnInit {
  private missionService = inject(MissionService);

  missions = signal<Mission[]>([]);

  async ngOnInit() {
    await this.loadMissions();
  }

  async loadMissions() {
    const missions = await this.missionService.getMissions();
    this.missions.set(missions);
  }
}
