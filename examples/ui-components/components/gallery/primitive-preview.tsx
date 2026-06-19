import { Badge } from "../ui/badge";
import { Button } from "../ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "../ui/card";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldSet,
} from "../ui/field";
import { Input } from "../ui/input";
import {
  Item,
  ItemActions,
  ItemContent,
  ItemDescription,
  ItemGroup,
  ItemMedia,
  ItemSeparator,
  ItemTitle,
} from "../ui/item";
import { Separator } from "../ui/separator";
import { Textarea } from "../ui/textarea";

export function PrimitivePreview() {
  return (
    <div className="ui-preview-grid">
      <Card id="button" className="ui-preview-card">
        <CardHeader>
          <CardTitle>Button and Badge</CardTitle>
          <CardDescription>Variants without CVA or Tailwind Merge.</CardDescription>
        </CardHeader>
        <CardContent className="ui-preview-cluster">
          <Button>Default</Button>
          <Button variant="outline">Outline</Button>
          <Button variant="secondary">Secondary</Button>
          <Badge>Ready</Badge>
          <Badge variant="outline">Forge receipt</Badge>
        </CardContent>
      </Card>
      <Card id="field" className="ui-preview-card">
        <CardHeader>
          <CardTitle>Field</CardTitle>
          <CardDescription>Form composition with source-owned labels.</CardDescription>
        </CardHeader>
        <CardContent>
          <FieldSet>
            <FieldGroup>
              <Field>
                <FieldLabel for="package-name">Package name</FieldLabel>
                <Input id="package-name" placeholder="ui/button" />
                <FieldDescription>Names stay stable in Forge receipts.</FieldDescription>
              </Field>
              <Field>
                <FieldLabel for="package-notes">Notes</FieldLabel>
                <Textarea id="package-notes" placeholder="Document provenance and runtime boundary." />
                <FieldError errors={[{ message: "Runtime imports must stay source-owned." }]} />
              </Field>
            </FieldGroup>
          </FieldSet>
        </CardContent>
      </Card>
      <Card id="item" className="ui-preview-card">
        <CardHeader>
          <CardTitle>Item</CardTitle>
          <CardDescription>List rows for registry and docs surfaces.</CardDescription>
        </CardHeader>
        <CardContent>
          <ItemGroup>
            <Item>
              <ItemMedia variant="icon">DX</ItemMedia>
              <ItemContent>
                <ItemTitle>Source-owned package</ItemTitle>
                <ItemDescription>No lifecycle scripts or node modules.</ItemDescription>
              </ItemContent>
              <ItemActions>
                <Badge variant="secondary">green</Badge>
              </ItemActions>
            </Item>
            <ItemSeparator />
            <Item variant="outline">
              <ItemContent>
                <ItemTitle>Adapter boundary</ItemTitle>
                <ItemDescription>Radix focus machines become DX-owned work.</ItemDescription>
              </ItemContent>
            </Item>
          </ItemGroup>
        </CardContent>
      </Card>
      <Card id="separator" className="ui-preview-card">
        <CardHeader>
          <CardTitle>Separator</CardTitle>
          <CardDescription>Visible structure without decorative clutter.</CardDescription>
        </CardHeader>
        <CardContent className="ui-separator-demo">
          <span>Registry</span>
          <Separator />
          <span>Forge</span>
          <Separator />
          <span>DX Check</span>
        </CardContent>
      </Card>
    </div>
  );
}
