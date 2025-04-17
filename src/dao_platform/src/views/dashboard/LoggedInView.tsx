import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { exploreDaos, userDaos } from "@/pages/Dashboard";
import { ExploreView } from "./ExploreView";
import { DaoCard } from "@/components/cards/DaoCard";

export const LoggedInView = () => {
    return (
        <Tabs defaultValue="my-daos" className="w-full">
            <TabsList className="mb-6">
                <TabsTrigger value="my-daos">My DAOs</TabsTrigger>
                <TabsTrigger value="explore">Explore</TabsTrigger>
            </TabsList>

            <TabsContent value="my-daos">
                <div className="mb-6">
                    <div className="flex justify-between items-center mb-4">
                        <h2 className="text-2xl font-bold">My DAOs</h2>
                        <Button>Create New DAO</Button>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {userDaos.map(dao => (
                            <DaoCard key={dao.uid} dao={dao} />
                        ))}
                    </div>
                </div>
            </TabsContent>

            <TabsContent value="explore">
                <ExploreView daos={exploreDaos} />
            </TabsContent>
        </Tabs>
    );
};