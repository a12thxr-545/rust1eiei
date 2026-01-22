import { inject, Injectable } from '@angular/core';
import { NgxSpinnerService } from 'ngx-spinner';
@Injectable({
  providedIn: 'root',
})
export class LoadingService {
  loadingRequestCount = 0
  private _spinnerService = inject(NgxSpinnerService)
  loading() {
    this.loadingRequestCount++
    this._spinnerService.show(undefined, {
      type: 'line-scale-pule-out-raapid',
      bdColor: 'rgb(0,0,0,0.8)',
      color: '#fff',
      fullScreen: false,
    })
  }
  idle() {
    this.loadingRequestCount--
    if (this.loadingRequestCount <= 0) {
      this.loadingRequestCount = 0
      this._spinnerService.hide()
    }
  }
}
