import { Dashboard } from "@/pages/Dashboard";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "./components/ui/toaster";
import { AuthenticationProvider } from "./providers/AuthenticationProvider";
import { PaddedLayout } from "./layouts/PaddedLayout";
import { DaoProvider } from "./providers/DaoProvider";

const queryClient = new QueryClient();

export const App = () => {
    return (
        <AuthenticationProvider>
            <QueryClientProvider client={queryClient}>
                <DaoProvider>
                    <PaddedLayout>
                        <Dashboard />
                    </PaddedLayout>
                </DaoProvider>
                <Toaster />
            </QueryClientProvider>
        </AuthenticationProvider>
    );
}
