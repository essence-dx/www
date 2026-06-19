type IconProps = {
  name: "bolt";
};

export function Icon({ name }: IconProps) {
  return <span aria-hidden="true" className={`template-icon template-icon--${name}`} />;
}
