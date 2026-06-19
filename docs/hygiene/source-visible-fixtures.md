# Source-Visible Fixture Ownership

This file is an audit contract for source-visible fixture and generated-proof roots.
These paths are allowed to exist only while they have an owner, a source, and a
removal gate. Removing a row without removing or relocating the matching path
reopens `source-visible-fixtures`.

Required row format:

```text
- `path/`: owner=team or area; source=why this path exists; removal_gate=condition for deleting or moving it
```

Owned roots:

- `.dx/template-app-browser-preview/`: owner=browser preview fixture; source=generated template app proof used by preview/runtime tests; removal_gate=tests stop reading the generated template app from the repo tree.
- `.dx/receipts/`: owner=receipt fixture contract; source=checked-in command receipts used as source-visible proof fixtures; removal_gate=receipt tests move to generated temp fixtures or serialized golden data.
- `components/`: owner=template fixture contract; source=Next-style fixture surface kept to prove source-visible app classification; removal_gate=fixture is moved under an explicit examples or tests fixture root.
- `lib/`: owner=template fixture contract; source=Next-style fixture helper surface kept to prove source-visible app classification; removal_gate=fixture is moved under an explicit examples or tests fixture root.
- `pages/`: owner=template fixture contract; source=Next-style fixture route surface kept to prove source-visible app classification; removal_gate=fixture is moved under an explicit examples or tests fixture root.
- `public/`: owner=template fixture contract; source=static asset fixture surface kept to prove source-visible app classification; removal_gate=fixture is moved under an explicit examples or tests fixture root.
- `dx-devtools/.dx/`: owner=devtools generated-proof contract; source=devtools output proof used to validate generated runtime and route artifacts; removal_gate=devtools proof tests stop reading checked-in `.dx` output.
