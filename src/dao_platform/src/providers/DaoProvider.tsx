import { Dao } from "@/models/entities/Dao";
import { DaoDiscoveryService } from "@/services/daoDiscoveryService";
import { useQuery } from "@tanstack/react-query";
import { createContext, ReactNode, useContext } from "react";
import { useAuthentication } from "./AuthenticationProvider";

export type DaoContextState = {
    exploreDaos: Dao[];
    userDaos: Dao[];
}

export const DaoContext = createContext<DaoContextState>({} as DaoContextState)

export const useDao = () => {
    const context = useContext(DaoContext)
    if (!context) {
        throw new Error("useDao must be used within a DaoProvider")
    }
    return context
}

export const DaoProvider = ({ children }: { children: ReactNode }) => {
    const { userPrincipal } = useAuthentication();

    const daoDiscoveryService = new DaoDiscoveryService();

    const exploreDaos = useQuery<Dao[]>({
        queryKey: ['exploreDaos'],
        queryFn: () => daoDiscoveryService.getRandomDaos()
    });

    const userDaos = useQuery<Dao[]>({
        queryKey: ['userDaos'],
        queryFn: () => daoDiscoveryService.getUserDaos(userPrincipal)
    });

    return (
        <DaoContext.Provider value={{ exploreDaos: exploreDaos.data || [], userDaos: userDaos.data || [] }}>
            {children}
        </DaoContext.Provider>
    )
}