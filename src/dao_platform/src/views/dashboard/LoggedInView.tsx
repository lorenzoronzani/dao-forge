import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useDao } from "@/providers/DaoProvider";
import { DaosView } from "./DaosView";
import { Button } from "@/components/ui/button";

export const LoggedInView = () => {
    const { userDaos, exploreDaos } = useDao();

    return (
        <Tabs defaultValue="my-daos" className="w-full">
            <div className="flex justify-between items-center mb-4">
                <TabsList className="mb-6">
                    <TabsTrigger value="my-daos">My DAOs</TabsTrigger>
                    <TabsTrigger value="explore">Explore</TabsTrigger>
                </TabsList>

                <Button>Create New DAO</Button>
            </div>

            <TabsContent value="my-daos">
                <DaosView title="My DAOs" daos={userDaos} />
            </TabsContent>

            <TabsContent value="explore">
                <DaosView title="Explore" daos={exploreDaos} />
            </TabsContent>
        </Tabs>
    );
};