import { cn } from "../../lib/utils";
import type { DxElementProps, DxInputProps } from "./types";

function InputOTP({ className, ...props }: DxInputProps) {
  return (
    <input
      inputmode="numeric"
      data-slot="input-otp"
      data-adapter-boundary="input-otp"
      className={cn("cn-input-otp", className)}
      {...props}
    />
  );
}

function InputOTPGroup({ className, ...props }: DxElementProps) {
  return <div data-slot="input-otp-group" className={cn("cn-input-otp-group", className)} {...props} />;
}

function InputOTPSlot({ className, index, ...props }: DxElementProps & { index?: number }) {
  return <span data-slot="input-otp-slot" data-index={index} className={cn("cn-input-otp-slot", className)} {...props} />;
}

function InputOTPSeparator({ className, ...props }: DxElementProps) {
  return <span data-slot="input-otp-separator" className={cn("cn-input-otp-separator", className)} {...props}>-</span>;
}

export { InputOTP, InputOTPGroup, InputOTPSlot, InputOTPSeparator };
