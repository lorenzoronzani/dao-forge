import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { VerticalLabeledComponent } from "@/components/labels/VerticalLabeledComponent";

type LocationInfoCardProps = {
    address: string;
    town: string;
    zip: number;
    onValueChange: (id: string, value: string | number) => void;
}

export const LocationInfoCard = ({ address, town, zip, onValueChange }: LocationInfoCardProps) => {
    return (
        <Card>
            <CardContent className="pt-6">
                <h3 className="text-lg font-medium mb-4">Location</h3>

                <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <VerticalLabeledComponent label="Address" htmlFor="address">
                        <Input
                            id="address"
                            name="address"
                            value={address}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Town" htmlFor="town">
                        <Input
                            id="town"
                            name="town"
                            value={town}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Zip Code" htmlFor="zip">
                        <Input
                            id="zip"
                            name="zip"
                            type="number"
                            value={zip}
                            onChange={(e) => onValueChange(e.target.name, Number(e.target.value))}
                            required
                        />
                    </VerticalLabeledComponent>
                </div>
            </CardContent>
        </Card>
    );
}