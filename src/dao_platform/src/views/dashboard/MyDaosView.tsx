import { DaoCard } from "@/components/cards/DaoCard"
import { Button } from "@/components/ui/button";
import { useDao } from "@/providers/DaoProvider";

export const MyDaosView = () => {
    const { userDaos } = useDao();

    return (<div className="mb-6">
        <div className="flex justify-between items-center mb-4">
            <h2 className="text-2xl font-bold">My DAOs</h2>
            <Button>Create New DAO</Button>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {userDaos.map(dao => (
                <DaoCard key={dao.uid} dao={dao} />
            ))}
        </div>
    </div>)
}