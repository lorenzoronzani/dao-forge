import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { exploreDaos } from "@/pages/Dashboard";
import { ExploreView } from "./ExploreView";
import { DaoCard } from "@/components/cards/DaoCard";
import { MyDaosView } from "./MyDaosView";

export const LoggedInView = () => {
    return (
        <Tabs defaultValue="my-daos" className="w-full">
            <TabsList className="mb-6">
                <TabsTrigger value="my-daos">My DAOs</TabsTrigger>
                <TabsTrigger value="explore">Explore</TabsTrigger>
            </TabsList>

            <TabsContent value="my-daos">
                <MyDaosView daos={exploreDaos} />
            </TabsContent>

            <TabsContent value="explore">
                <ExploreView daos={exploreDaos} />
            </TabsContent>
        </Tabs>
    );
};