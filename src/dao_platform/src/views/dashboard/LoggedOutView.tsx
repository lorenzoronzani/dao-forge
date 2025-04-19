import { Button } from "@/components/ui/button";
import { useAuthentication } from "@/providers/AuthenticationProvider";
import { ExploreView } from "./ExploreView";

export const LoggedOutView = () => {
    const { login } = useAuthentication();

    return (
        <div>
            <div className="text-center mb-10">
                <h2 className="text-3xl font-bold mb-4">Discover and Join DAOs on Internet Computer</h2>
                <p className="text-slate-600 max-w-2xl mx-auto">
                    Create, manage, and participate in Decentralized Autonomous Organizations built on Internet Computer Protocol.
                </p>
                <div className="mt-6">
                    <Button size="lg" onClick={login}>Get Started</Button>
                </div>
            </div>

            <ExploreView />
        </div>
    );
};