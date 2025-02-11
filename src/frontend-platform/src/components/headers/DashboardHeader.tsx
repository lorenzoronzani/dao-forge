import { Plus } from "lucide-react";
import { Button } from "@/components/ui/button";

export const DashboardHeader = () => {
    return (
        <div className="flex justify-between items-center mb-8">
            <div>
                <h1 className="text-3xl font-bold">My DAOs</h1>
                <p className="text-gray-500 mt-2">Manage your decentralized organizations</p>
            </div>
            <Button className="flex items-center gap-2">
                <Plus className="w-4 h-4" />
                Create New DAO
            </Button>
        </div>
    );
};