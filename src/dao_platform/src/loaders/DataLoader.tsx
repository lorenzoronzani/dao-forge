import { Skeleton } from "@/components/ui/skeleton";
import TopBar from "@/components/headers/TopBar";
import { MainContainer } from "@/layouts/MainContainer";
import React, { ReactNode } from "react";
import { useDao } from "@/providers/DaoProvider";

interface DataLoaderProps {
    children: ReactNode;
}

export const DataLoader: React.FC<DataLoaderProps> = ({ children }) => {
    const { isLoading, userDaos, exploreDaos } = useDao()

    if (isLoading && (userDaos.length === 0 || exploreDaos.length === 0)) {
        return (
            <>
                <TopBar />
                <MainContainer>
                    <div className="flex flex-col space-y-4 animate-pulse">
                        <Skeleton className="h-8 w-24 bg-gray-300" />
                        <div className="space-y-2">
                            <Skeleton className="h-4 w-3/4 bg-gray-300" />
                            <Skeleton className="h-4 w-1/2 bg-gray-300" />
                        </div>
                        <div className="flex space-x-2">
                            <Skeleton className="h-10 w-24 bg-gray-300" />
                            <Skeleton className="h-10 w-24 bg-gray-300" />
                            <Skeleton className="h-10 w-24 bg-gray-300" />
                            <Skeleton className="h-10 w-24 bg-gray-300" />
                        </div>
                    </div>
                </MainContainer>
            </>
        );
    }

    return <>{children}</>;
};