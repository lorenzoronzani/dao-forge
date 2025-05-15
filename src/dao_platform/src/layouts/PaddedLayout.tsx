import { ReactNode } from "react";

type PaddedLayoutProps = {
    children: ReactNode;
}

export const PaddedLayout = ({ children }: PaddedLayoutProps) => {
    return (
        <div className="min-h-screen bg-slate-50">
            {children}
        </div>
    );
};