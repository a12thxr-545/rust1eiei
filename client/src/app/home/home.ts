import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { PassportService } from '../_services/passport-service';

@Component({
  selector: 'app-home',
  imports: [],
  templateUrl: './home.html',
  styleUrl: './home.css',
})
export class Home {
  private router = inject(Router);
  private passport = inject(PassportService);


}
