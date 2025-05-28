import { Dao } from "@/models/entities/Dao";
import { DaoDiscoveryService } from "@/services/daoDiscoveryService";
import { useQuery } from "@tanstack/react-query";
import { createContext, ReactNode, useContext } from "react";
import { useAuthentication } from "./AuthenticationProvider";
import { Principal } from "@dfinity/principal";

export type DaoContextState = {
    exploreDaos: Dao[];
    userDaos: Dao[];
    refreshData: () => void;
    getDao: (daoPrincipal: Principal) => Dao;
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
    const { userPrincipal, identity } = useAuthentication();

    const daoDiscoveryService = new DaoDiscoveryService(identity);

    const exploreDaos = useQuery<Dao[]>({
        queryKey: ['exploreDaos'],
        queryFn: () => daoDiscoveryService.getRandomDaos(),
        refetchInterval: 60 * 1000,
    });

    const userDaos = useQuery<Dao[]>({
        queryKey: ['userDaos'],
        queryFn: () => daoDiscoveryService.getUserDaos(userPrincipal),
        refetchInterval: 60 * 1000,
    });

    const refreshData = () => {
        exploreDaos.refetch();
        userDaos.refetch();
    }

    const findDaoByPrincipal = (daos: Dao[], principal: Principal): Dao | undefined => {
        return daos.filter((dao) => dao.principal.toText() === principal.toText())[0];
    }

    const getDao = (daoPrincipal: Principal): Dao => {
        let dao = findDaoByPrincipal(userDaos.data || [], daoPrincipal);

        if (!dao) {
            dao = findDaoByPrincipal(exploreDaos.data || [], daoPrincipal);
        }

        if (!dao) {
            throw new Error("Dao not found");
        }

        return dao;
    }

    return (
        <DaoContext.Provider value={{ exploreDaos: exploreDaos.data || [], userDaos: userDaos.data || [], refreshData, getDao }}>
            {children}
        </DaoContext.Provider>
    )
}