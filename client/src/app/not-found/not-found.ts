import { Component, AfterViewInit, OnDestroy } from '@angular/core';
import { Router, RouterLink } from '@angular/router';

declare var Snap: any;
declare var mina: any;

@Component({
  selector: 'app-not-found',
  standalone: true,
  imports: [RouterLink],
  templateUrl: './not-found.html',
  styleUrl: './not-found.css',
})
export class NotFound implements AfterViewInit, OnDestroy {
  private intervalId: any;

  constructor(private router: Router) { }

  returnToBase() {
    this.router.navigate(['/']);
  }

  ngAfterViewInit() {
    // Check if Snap is available (loaded from CDN or assets)
    if (typeof Snap !== 'undefined') {
      this.initAnimation();
    }
  }

  ngOnDestroy() {
    if (this.intervalId) {
      clearInterval(this.intervalId);
    }
  }

  private initAnimation() {
    const monkeySVG = Snap('#monkey_404');
    if (!monkeySVG) return;

    const tail = monkeySVG.select('#tail');
    if (!tail) return;

    const revert = () => {
      tail.animate(
        {
          d: 'M89,315c2.2-15.2-23-13.2-21.6,4.8c1.7,22.3,24.4,22.1,42.5,9.1c10.8-7.8,15.3-1.8,19.1,1.1 c2.3,1.7,6.7,3.3,11-3'
        },
        1600,
        mina.easeinout
      );
    };

    this.intervalId = setInterval(() => {
      tail.animate(
        {
          d: 'M81,310c-8.8-6.5-20.8,6.5-15,18c7.4,14.5,22.5,10.8,31,3c9.8-9,18.9-5.6,22-2 c5.8,6.8,16.7,4.3,21-2'
        },
        1600,
        mina.easeinout,
        revert
      );
    }, 3200);
  }
}
