export type DxElementProps = {
  className?: string;
  children?: any;
  [attribute: string]: any;
};

export type DxInputProps = DxElementProps & {
  type?: string;
  value?: string;
  name?: string;
  placeholder?: string;
};

export type DxTextareaProps = DxElementProps & {
  value?: string;
  name?: string;
  placeholder?: string;
  rows?: number;
};
