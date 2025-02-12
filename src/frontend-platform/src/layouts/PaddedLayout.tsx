import { ReactNode } from "react";

type PaddedLayoutProps = {
    children: ReactNode;
}

export const PaddedLayout = ({ children }: PaddedLayoutProps) => {
    return (
        <div className="p-8">
            {children}
        </div>
    );
};