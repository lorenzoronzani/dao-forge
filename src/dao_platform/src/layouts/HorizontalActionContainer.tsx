import { ReactNode } from "react";

type HorizontalActionContainerProps = {
    children: ReactNode;
};

export const HorizontalActionContainer = ({ children }: HorizontalActionContainerProps) => {
    return (
        <div className="flex justify-end gap-4">
            {children}
        </div>
    );
}
