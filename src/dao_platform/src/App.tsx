import { Dashboard } from "@/pages/Dashboard";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Toaster } from "./components/ui/toaster";
import { AuthenticationProvider } from "./providers/AuthenticationProvider";
import { PaddedLayout } from "./layouts/PaddedLayout";

const queryClient = new QueryClient();

export const App = () => {
    return (
        <AuthenticationProvider>
            <QueryClientProvider client={queryClient}>
                <PaddedLayout>
                    <Dashboard />
                </PaddedLayout>
                <Toaster />
            </QueryClientProvider>
        </AuthenticationProvider>
    );
}
