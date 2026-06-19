export type NumberFieldProps = {
  readonly label: string;
  readonly min?: number;
  readonly onInput: (value: number) => void;
  readonly value: number;
};

export function NumberField({
  label,
  min,
  onInput,
  value,
}: NumberFieldProps) {
  return (
    <label className="wb-number-field">
      <span>{label}</span>
      <input
        min={min}
        onInput={(event) => onInput(Number(event.currentTarget.value))}
        step="1"
        type="number"
        value={Math.round(value)}
      />
    </label>
  );
}
