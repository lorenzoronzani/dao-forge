import { Dashboard } from "@/pages/Dashboard";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const queryClient = new QueryClient();

export const App = () => {
    return (
        <QueryClientProvider client={queryClient}>
            <Dashboard />
        </QueryClientProvider>
    );
}
