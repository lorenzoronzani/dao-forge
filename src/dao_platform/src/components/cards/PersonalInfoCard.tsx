import { VerticalLabeledComponent } from "../labels/VerticalLabeledComponent";
import { Card, CardContent } from "../ui/card";
import { Input } from "../ui/input";

type PersonalInfoCardProps = {
    userFirstName: string;
    userLastName: string;
    userAddress: string;
    userTown: string;
    userZip: number;
    userPhone: string;
    userEmail: string;
    onValueChange: (id: string, value: string | number) => void;
}

export const PersonalInfoCard = ({ userFirstName, userLastName, userAddress, userTown, userZip, userPhone, userEmail, onValueChange }: PersonalInfoCardProps) => {
    return (
        <Card>
            <CardContent className="pt-6">
                <h3 className="text-lg font-medium mb-4">Personal Information</h3>

                <div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
                    <VerticalLabeledComponent label="Name" htmlFor="userFirstName">
                        <Input
                            id="userFirstName"
                            name="userFirstName"
                            value={userFirstName}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Surname" htmlFor="userLastName">
                        <Input
                            id="userLastName"
                            name="userLastName"
                            value={userLastName}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Address" htmlFor="userAddress">
                        <Input
                            id="userAddress"
                            name="userAddress"
                            value={userAddress}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Town" htmlFor="userTown">
                        <Input
                            id="userTown"
                            name="userTown"
                            value={userTown}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Zip Code" htmlFor="userZip">
                        <Input
                            id="userZip"
                            name="userZip"
                            type="number"
                            value={userZip}
                            onChange={(e) => onValueChange(e.target.name, Number(e.target.value))}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Phone" htmlFor="userPhone">
                        <Input
                            id="userPhone"
                            name="userPhone"
                            value={userPhone}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>

                    <VerticalLabeledComponent label="Email" htmlFor="userEmail">
                        <Input
                            id="userEmail"
                            name="userEmail"
                            value={userEmail}
                            onChange={(e) => onValueChange(e.target.name, e.target.value)}
                            required
                        />
                    </VerticalLabeledComponent>
                </div>
            </CardContent>
        </Card>
    )
}