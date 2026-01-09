import { AbstractControl, ValidationErrors, ValidatorFn } from "@angular/forms";

export const passwordValidator = (min: number, max: number): ValidatorFn => {
    return (control: AbstractControl): ValidationErrors | null => {
        const password = control.value as string;
        if (!password) return { required: true };
        if (password.length < min || password.length > max) return { invalidLength: true };
        if (!/[a-z]/.test(password)) return { invalidLowerCase: true };
        if (!/[A-Z]/.test(password)) return { invalidUpperCase: true };
        if (!/[0-9]/.test(password)) return { invalidNumeric: true };
        if (!/[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>/?]/.test(password)) return { invalidSpecialChar: true };
        return null;
    }
}

export const PasswordMatchValidator = (ctrl_pw_name: string, ctrl_cf_pw_name: string): ValidatorFn => {
    return (formGroup: AbstractControl) => {
        const ctrlPw = formGroup.get(ctrl_pw_name)?.value;
        const ctrlCfPw = formGroup.get(ctrl_cf_pw_name);
        if (!ctrlPw || !ctrlCfPw) return null;
        const isMatch = ctrlPw === ctrlCfPw.value;
        if (!isMatch) ctrlCfPw.setErrors({ mismatch: true });
        else ctrlCfPw.setErrors(null);
        return null;
    }
}