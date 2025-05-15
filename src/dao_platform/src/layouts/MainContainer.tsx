import { ReactNode } from "react";

type MainContainerProps = {
    children: ReactNode;
};

export const MainContainer = ({ children }: MainContainerProps) => {
    return (
        <main className="container mx-auto px-4 py-8">
            {children}
        </main>
    );
}
