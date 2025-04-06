import { Dashboard } from "@/pages/Dashboard";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import TopBar from "./components/headers/TopBar";
import { Toaster } from "./components/ui/toaster";
import { AuthenticationProvider } from "./providers/AuthenticationProvider";

const queryClient = new QueryClient();

export const App = () => {
    return (
        <AuthenticationProvider>
            <QueryClientProvider client={queryClient}>
                <TopBar />
                <Dashboard />
                <Toaster />
            </QueryClientProvider>
        </AuthenticationProvider>
    );
}
