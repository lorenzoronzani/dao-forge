import { ReactNode } from "react";

interface HorizontalLabeledFieldProps {
    image: ReactNode;
    label: string;
    text: string;
}

export const HorizontalLabeledField = ({ image, label, text }: HorizontalLabeledFieldProps) => {
    return (
        <div className="flex items-center gap-1">
            {image}
            <span className="text-slate-500 text-sm">{label}</span>
            <span className="text-sm ml-1">{text}</span>
        </div>
    )
}