import { Label } from "@/components/ui/label";
import { ReactNode } from "react";

type VerticalLabeledComponentProps = {
    label: string;
    htmlFor: string;
    children: ReactNode;
}

export const VerticalLabeledComponent = ({ label, htmlFor, children }: VerticalLabeledComponentProps) => {
    return (
        <div>
            <Label htmlFor={htmlFor} className="mb-2 block">{label}</Label>
            {children}
        </div >
    )
}