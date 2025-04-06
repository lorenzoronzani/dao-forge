import { getIdentityProvider } from "@/utils/identityProvider";
import { AuthClient } from "@dfinity/auth-client";
import { Principal } from "@dfinity/principal";
import { createContext, ReactNode, useContext, useEffect, useState } from "react";

export type AuthenticationContextState = {
    isAuthenticated: boolean;
    userPrincipal: Principal;
    login: () => Promise<void>;
    logout: () => Promise<void>;
}

export const AuthenticationContext = createContext<AuthenticationContextState>({} as AuthenticationContextState)

export const useAuthentication = () => {
    const context = useContext(AuthenticationContext)
    if (!context) {
        throw new Error("useAuthentication must be used within an AuthenticationProvider")
    }
    return context
}

export const AuthenticationProvider = ({ children }: { children: ReactNode }) => {
    const [isAuthenticated, setIsAuthenticated] = useState<boolean>(false);
    const [authClient, setAuthClient] = useState<AuthClient | null>(null);
    const [userPrincipal, setUserPrincipal] = useState<Principal>(Principal.anonymous());

    const identityProvider = getIdentityProvider();

    useEffect(() => {
        init();
    }, []);

    const init = async () => {
        try {
            setAuthClient(await AuthClient.create());
        } catch (error) {
            console.error(error);
        }
    }

    const login = async () => {
        try {
            authClient?.login({
                identityProvider,
                onSuccess: async () => {
                    setUserPrincipal(authClient?.getIdentity().getPrincipal());
                    setIsAuthenticated(await authClient?.isAuthenticated());
                },
                onError: (error) => {
                    console.error(error);
                }
            });
        } catch (error) {
            console.error(error);
        }
    };

    const logout = async () => {
        try {
            authClient?.logout();
            setUserPrincipal(Principal.anonymous());
            setIsAuthenticated(false);
        } catch (error) {
            console.error(error);
        }
    };

    return (
        <AuthenticationContext.Provider value={{ isAuthenticated, userPrincipal, login, logout }}>
            {children}
        </AuthenticationContext.Provider>
    );
}