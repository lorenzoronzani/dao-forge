import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "./components/ui/toaster";
import { AuthenticationProvider } from "./providers/AuthenticationProvider";
import { PaddedLayout } from "./layouts/PaddedLayout";
import { DaoProvider } from "./providers/DaoProvider";
import { Provider } from 'react-redux';
import { persistor, store } from './store';
import { PersistGate } from "redux-persist/integration/react";
import { Outlet } from "react-router";

const queryClient = new QueryClient();

export const App = () => {
    return (
        <Provider store={store}>
            <PersistGate loading={<div>Loading...</div>} persistor={persistor}>
                <AuthenticationProvider>
                    <QueryClientProvider client={queryClient}>
                        <DaoProvider>
                            <PaddedLayout>
                                <Outlet />
                            </PaddedLayout>
                        </DaoProvider>
                        <Toaster />
                    </QueryClientProvider>
                </AuthenticationProvider>
            </PersistGate>
        </Provider>
    );
}
