import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { ExploreView } from "./ExploreView";
import { MyDaosView } from "./MyDaosView";

export const LoggedInView = () => {
    return (
        <Tabs defaultValue="my-daos" className="w-full">
            <TabsList className="mb-6">
                <TabsTrigger value="my-daos">My DAOs</TabsTrigger>
                <TabsTrigger value="explore">Explore</TabsTrigger>
            </TabsList>

            <TabsContent value="my-daos">
                <MyDaosView />
            </TabsContent>

            <TabsContent value="explore">
                <ExploreView />
            </TabsContent>
        </Tabs>
    );
};