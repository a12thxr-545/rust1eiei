import { Component, inject, Input, NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { NgxSpinnerService } from './spinner.service';
export { NgxSpinnerService } from './spinner.service';

@Component({
    selector: 'ngx-spinner',
    standalone: true,
    imports: [CommonModule],
    template: `
    @if (spinnerService.isVisible(name)()) {
      <div class="ngx-spinner-overlay">
        <div class="ngx-spinner-container">
          <div class="ngx-spinner-ring"></div>
        </div>
      </div>
    }
  `,
    styles: [`
    .ngx-spinner-overlay {
      position: absolute;
      inset: 0;
      display: flex;
      align-items: center;
      justify-content: center;
      background: rgba(0,0,0,0.7);
      border-radius: inherit;
      z-index: 9999;
    }
    .ngx-spinner-ring {
      width: 40px;
      height: 40px;
      border: 3px solid rgba(255,255,255,0.2);
      border-top-color: #fff;
      border-radius: 50%;
      animation: spin 0.7s linear infinite;
    }
    @keyframes spin {
      to { transform: rotate(360deg); }
    }
  `]
})
export class NgxSpinnerComponent {
    @Input() name: string = 'primary';
    @Input() bdColor: string = 'rgba(0,0,0,0.8)';
    @Input() type: string = '';
    @Input() size: string = 'medium';
    spinnerService = inject(NgxSpinnerService);
}

@NgModule({
    imports: [NgxSpinnerComponent],
    exports: [NgxSpinnerComponent]
})
export class NgxSpinnerModule { }
