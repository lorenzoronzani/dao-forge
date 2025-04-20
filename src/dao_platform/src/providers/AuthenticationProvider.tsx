import { useAppDispatch, useAppSelector } from "@/hooks/redux";
import { login, logout } from "@/store/authenticationSlice";
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
    const dispatch = useAppDispatch();
    const { isAuthenticated, userPrincipal } = useAppSelector((state) => state.authentication);

    const [authClient, setAuthClient] = useState<AuthClient | null>(null);
    const [user, setUser] = useState<Principal>(Principal.from(userPrincipal));

    const identityProvider = getIdentityProvider();

    useEffect(() => {
        init();
    }, []);

    const init = async () => {
        try {
            const client = await AuthClient.create();
            setAuthClient(client);

            if (await client.isAuthenticated()) {
                dispatch(login(client.getIdentity().getPrincipal()!.toText()));
                setUser(client.getIdentity().getPrincipal()!);
            } else {
                dispatch(logout());
                setUser(Principal.anonymous());
            }
        } catch (error) {
            console.error(error);
        }
    }

    const handleLogin = async () => {
        try {
            authClient?.login({
                identityProvider,
                onSuccess: async () => {
                    dispatch(login(authClient?.getIdentity().getPrincipal()!.toText()));
                    setUser(authClient?.getIdentity().getPrincipal()!);
                },
                onError: (error) => {
                    console.error(error);
                }
            });
        } catch (error) {
            console.error(error);
        }
    };

    const handleLogout = async () => {
        try {
            authClient?.logout();
            dispatch(logout());
            setUser(Principal.anonymous());
        } catch (error) {
            console.error(error);
        }
    };

    return (
        <AuthenticationContext.Provider value={{ isAuthenticated, userPrincipal: user, login: handleLogin, logout: handleLogout }}>
            {children}
        </AuthenticationContext.Provider>
    );
}