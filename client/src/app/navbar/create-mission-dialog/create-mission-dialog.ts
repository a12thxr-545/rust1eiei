import { Component, ElementRef, inject, signal, viewChild } from '@angular/core';
import { FormControl, FormGroup, ReactiveFormsModule, Validators } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogRef, MatDialogModule } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MissionService } from '../../_services/mission-service';
import { SnackbarService } from '../../_services/snackbar.service';
import { NgxSpinnerService } from 'ngx-spinner';

@Component({
  selector: 'app-create-mission-dialog',
  imports: [
    ReactiveFormsModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatDialogModule
  ],
  templateUrl: './create-mission-dialog.html',
  styleUrl: './create-mission-dialog.css'
})
export class CreateMissionDialog {
  private dialogRef = inject(MatDialogRef<CreateMissionDialog>);
  private missionService = inject(MissionService);
  private snackbarService = inject(SnackbarService);
  private spinnerService = inject(NgxSpinnerService);

  form = new FormGroup({
    name: new FormControl('', [Validators.required, Validators.minLength(3)]),
    description: new FormControl('')
  });

  isLoading = signal(false);
  imagePreview = signal<string | null>(null);
  selectedFile: File | null = null;
  imageInput = viewChild<ElementRef<HTMLInputElement>>('imageInput');

  triggerImageInput() {
    this.imageInput()?.nativeElement.click();
  }

  onImageSelected(event: Event) {
    const input = event.target as HTMLInputElement;
    if (input.files && input.files[0]) {
      this.selectedFile = input.files[0];
      const reader = new FileReader();
      reader.onload = (e) => {
        this.imagePreview.set(e.target?.result as string);
      };
      reader.readAsDataURL(this.selectedFile);
    }
  }

  async onSubmit() {
    if (this.form.invalid) return;

    this.isLoading.set(true);
    this.spinnerService.show('mission-spinner');

    try {
      let imageUrl: string | undefined;

      if (this.imagePreview()) {
        const uploadedImage = await this.missionService.uploadMissionImage(this.imagePreview()!);
        if (uploadedImage) {
          imageUrl = uploadedImage.url;
        }
      }

      const error = await this.missionService.createMission({
        name: this.form.controls.name.value!,
        description: this.form.controls.description.value || undefined,
        image_url: imageUrl
      });

      if (error) {
        this.snackbarService.error(error);
        return;
      }

      this.snackbarService.success('Mission created successfully');
      this.dialogRef.close(true);
    } catch (error) {
      this.snackbarService.error('An unexpected error occurred');
    } finally {
      this.isLoading.set(false);
      this.spinnerService.hide('mission-spinner');
    }
  }

  onCancel() {
    this.dialogRef.close();
  }
}
